#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include "qgridlayout.h"
#include <QDebug>
#define QD qDebug() << __FILE__ << __LINE__
#include <QMainWindow>

QT_BEGIN_NAMESPACE
namespace Ui {
class MainWindow;
}
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void on_actionOpen_Gif_triggered();

private:
    Ui::MainWindow *ui;
    QString m_path;
    QGridLayout *m_layout;
};
#endif // MAINWINDOW_H
