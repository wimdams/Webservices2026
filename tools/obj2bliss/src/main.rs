use std::{collections::BTreeMap, env, error::Error, fs, io::Write};

type Vertex = [f32; 3];
type Edge = (u32, u32);

fn normal(a: Vertex, b: Vertex, c: Vertex) -> Vertex {
    let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];

    let n = [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ];

    let len = n.iter().map(|x| x * x).sum::<f32>().sqrt();
    if len == 0.0 {
        return [0.0; 3];
    }

    n.map(|x| x / len)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        return Err("Usage: obj2bliss <input.obj> <output.bliss>".into());
    }

    let input = fs::read_to_string(&args[1])?;

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut faces: Vec<[u32; 3]> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();

        match parts.next() {
            Some("v") => {
                let coords = parts
                    .take(3)
                    .map(str::parse::<f32>)
                    .collect::<Result<Vec<_>, _>>()?;

                if coords.len() != 3 || coords.iter().any(|x| !x.is_finite()) {
                    return Err("Invalid vertex".into());
                }

                vertices.push([coords[0], coords[1], coords[2]]);
            }

            Some("f") => {
                let indices = parts
                    .map(|part| {
                        let index = part.split('/').next().unwrap();
                        index.parse::<u32>().map(|n| n - 1)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                if indices.len() != 3 {
                    return Err("Only triangular faces supported".into());
                }

                if indices.iter().any(|&i| i as usize >= vertices.len()) {
                    return Err("Invalid vertex index".into());
                }

                faces.push([indices[0], indices[1], indices[2]]);
            }

            _ => {}
        }
    }

    let mut edges: BTreeMap<Edge, Vec<Vertex>> = BTreeMap::new();

    for [a, b, c] in faces {
        let n = normal(
            vertices[a as usize],
            vertices[b as usize],
            vertices[c as usize],
        );

        for (a, b) in [(a, b), (b, c), (c, a)] {
            if a == b {
                continue;
            }

            let edge = (a.min(b), a.max(b));
            edges.entry(edge).or_default().push(n);
        }
    }

    edges.retain(|_, normals| {
        if normals.len() != 2 {
            return true;
        }

        let dot: f32 = normals[0]
            .iter()
            .zip(normals[1].iter())
            .map(|(a, b)| a * b)
            .sum();

        dot.abs() < 0.9999
    });

    if !vertices.is_empty() {
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];

        for v in &vertices {
            for i in 0..3 {
                min[i] = min[i].min(v[i]);
                max[i] = max[i].max(v[i]);
            }
        }

        let center = std::array::from_fn::<_, 3, _>(|i| (min[i] + max[i]) / 2.0);

        let size = (0..3).map(|i| max[i] - min[i]).fold(0.0_f32, f32::max);

        for v in &mut vertices {
            for i in 0..3 {
                v[i] = if size > 0.0 {
                    2.0 * (v[i] - center[i]) / size
                } else {
                    0.0
                };
            }
        }
    }

    let mut file = fs::File::create(&args[2])?;

    file.write_all(b"BLIS")?;
    file.write_all(&[1])?;
    file.write_all(&[0; 3])?;

    file.write_all(&(vertices.len() as u32).to_le_bytes())?;
    file.write_all(&(edges.len() as u32).to_le_bytes())?;

    for vertex in &vertices {
        for coordinate in vertex {
            file.write_all(&coordinate.to_le_bytes())?;
        }
    }

    for (a, b) in edges.keys() {
        file.write_all(&a.to_le_bytes())?;
        file.write_all(&b.to_le_bytes())?;
    }

    println!("{} vertices, {} edges", vertices.len(), edges.len());

    Ok(())
}
