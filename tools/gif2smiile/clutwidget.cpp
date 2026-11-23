#include "clutwidget.h"

#include <QPainter>

CLUTWidget::CLUTWidget(QWidget *parent)
    : QWidget{parent}
{
    this->setMinimumSize(12*11,12*27);
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
    for(int y = 0; y < 26; y++){
        for(int x = 0; x < 10; x++){
            if(m_colors.length() != 0 && i < m_colors.length()){
                p.setBrush(m_colors.at(i));
            }else{
                p.setBrush(Qt::black);
            }
            p.drawRect(x*12, y*12, 10, 10);
            i++;
        }
    }
}
