use crate::framebuffer::Framebuffer;

const HEADER_SIZE: usize = 16;
const VERTEX_SIZE: usize = 12;
const EDGE_SIZE: usize = 8;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy)]
pub struct Edge {
    pub a: u32,
    pub b: u32,
}

pub struct Bliss<'a> {
    data: &'a [u8],
    vertex_count: usize,
    edge_count: usize,
}

#[derive(Debug)]
pub enum BlissError {
    InvalidHeader,
    InvalidVertex,
    InvalidEdge,
    DuplicateEdge,
}

fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
    let bytes: [u8; 4] = data.get(offset..offset.checked_add(4)?)?.try_into().ok()?;
    Some(u32::from_le_bytes(bytes))
}

fn read_f32(data: &[u8], offset: usize) -> Option<f32> {
    Some(f32::from_bits(read_u32(data, offset)?))
}

impl<'a> Bliss<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, BlissError> {
        if data.get(0..4) != Some(b"BLIS")
            || data.get(4) != Some(&1)
            || data.get(5..8) != Some(&[0, 0, 0][..])
        {
            return Err(BlissError::InvalidHeader);
        }

        let vertex_count = read_u32(data, 8).ok_or(BlissError::InvalidHeader)? as usize;
        let edge_count = read_u32(data, 12).ok_or(BlissError::InvalidHeader)? as usize;

        let expected_len = vertex_count
            .checked_mul(VERTEX_SIZE)
            .and_then(|n| {
                edge_count
                    .checked_mul(EDGE_SIZE)
                    .and_then(|e| HEADER_SIZE.checked_add(n)?.checked_add(e))
            })
            .ok_or(BlissError::InvalidHeader)?;

        if data.len() != expected_len {
            return Err(BlissError::InvalidHeader);
        }

        let model = Self {
            data,
            vertex_count,
            edge_count,
        };

        for i in 0..vertex_count {
            let v = model.vertex(i).ok_or(BlissError::InvalidVertex)?;

            for c in [v.x, v.y, v.z] {
                if !c.is_finite() || !(-1.0..=1.0).contains(&c) {
                    return Err(BlissError::InvalidVertex);
                }
            }
        }

        for i in 0..edge_count {
            let e = model.edge(i).ok_or(BlissError::InvalidEdge)?;

            if e.a >= e.b || e.b as usize >= vertex_count {
                return Err(BlissError::InvalidEdge);
            }

            for j in 0..i {
                let previous = model.edge(j).ok_or(BlissError::InvalidEdge)?;
                if e.a == previous.a && e.b == previous.b {
                    return Err(BlissError::DuplicateEdge);
                }
            }
        }

        Ok(model)
    }

    pub fn vertex(&self, index: usize) -> Option<Vertex> {
        if index >= self.vertex_count {
            return None;
        }

        let offset = HEADER_SIZE + index * VERTEX_SIZE;

        Some(Vertex {
            x: read_f32(self.data, offset)?,
            y: read_f32(self.data, offset + 4)?,
            z: read_f32(self.data, offset + 8)?,
        })
    }

    pub fn edge(&self, index: usize) -> Option<Edge> {
        if index >= self.edge_count {
            return None;
        }

        let offset = HEADER_SIZE + self.vertex_count * VERTEX_SIZE + index * EDGE_SIZE;

        Some(Edge {
            a: read_u32(self.data, offset)?,
            b: read_u32(self.data, offset + 4)?,
        })
    }

    pub(crate) unsafe fn from_validated(data: &'a [u8]) -> Self {
        Self {
            data,
            vertex_count: read_u32(data, 8).unwrap() as usize,
            edge_count: read_u32(data, 12).unwrap() as usize,
        }
    }
}

fn project(vertex: Vertex, sin_angle: f32, cos_angle: f32) -> (usize, usize) {
    let x = vertex.x * cos_angle + vertex.z * sin_angle;
    let z = -vertex.x * sin_angle + vertex.z * cos_angle;
    let y = vertex.y;

    const ELEVATION: f32 = 25.0_f32.to_radians();

    let sin_elev = libm::sinf(ELEVATION);
    let cos_elev = libm::cosf(ELEVATION);

    let projected_y = y * cos_elev - z * sin_elev;

    let scale = 0.65;

    let sx = 119.5 + x * scale * 119.5;
    let sy = 135.5 - projected_y * scale * 135.5;

    (sx.clamp(0.0, 239.0) as usize, sy.clamp(0.0, 271.0) as usize)
}

pub fn render(fb: &mut Framebuffer, model: &Bliss, angle: f32, color: u16) {
    let sin_angle = libm::sinf(angle);
    let cos_angle = libm::cosf(angle);

    for i in 0..model.edge_count {
        let Some(edge) = model.edge(i) else {
            continue;
        };

        let Some(a) = model.vertex(edge.a as usize) else {
            continue;
        };
        let Some(b) = model.vertex(edge.b as usize) else {
            continue;
        };

        let (x0, y0) = project(a, sin_angle, cos_angle);
        let (x1, y1) = project(b, sin_angle, cos_angle);

        fb.draw_line(
            x0, y0, x1, y1, 1,   // Thickness
            240, // Left-half width
            272, // Screen height
            color,
        );
    }
}
