#ifndef CLUTWIDGET_H
#define CLUTWIDGET_H

#include <QWidget>

class CLUTWidget : public QWidget
{
    Q_OBJECT
public:
    explicit CLUTWidget(QWidget *parent = nullptr);
    void setCLUT(QList<QRgb> colors);
    void paintEvent(QPaintEvent *event) override;
private:
    QList<QRgb> m_colors;

signals:

};

#endif // CLUTWIDGET_H
