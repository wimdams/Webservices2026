#include "clutwidget.h"

#include <QMouseEvent>
#include <QPainter>

CLUTWidget::CLUTWidget(QWidget *parent)
    : QWidget{parent}
{
    this->setMinimumSize(12*11,12*27+100);
    this->setMouseTracking(true);
}

void CLUTWidget::setCLUT(QList<QRgb> colors)
{
    m_colors = colors;
    update();
}

void CLUTWidget::paintEvent(QPaintEvent *event)
{
    QPainter p(this);
    int i = 0;
    int index = -1;
    QRgb color;
    for(int y = 0; y < 26; y++){
        for(int x = 0; x < 10; x++){
            if(m_colors.length() != 0 && i < m_colors.length()){
                p.setBrush(m_colors.at(i));
            }else{
                p.setBrush(Qt::black);
            }
            p.drawRect(x*12, y*12, 10, 10);

            if(m_pos.x() > (x*12) && m_pos.x() < (x*12 + 10)){
                if(m_pos.y() > (y*12) && m_pos.y() < (y*12 + 10)){
                    if(m_colors.length() != 0 && i < m_colors.length()){
                        index = i;
                        color = m_colors.at(i);
                    }
                }
            }

            i++;
        }
    }
    if(index!=-1){
        p.drawText(1,26*12+10,tr("Color: 0x%1 @ %2")
            .arg((quint32)(color & 0xFFFFFF),6,16,QChar('0'))
            .arg(index));
    }
}

void CLUTWidget::mouseMoveEvent(QMouseEvent *event)
{
    m_pos = event->pos(); //Deprecated function!
    update();
}
