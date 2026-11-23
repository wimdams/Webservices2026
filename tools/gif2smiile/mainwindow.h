#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QDebug>
#define QD qDebug() << __FILE__ << __LINE__
#include <QMainWindow>
#include <QFile>
#include <QFileDialog>
#include <QImageReader>
#include <QLabel>
#include <QMessageBox>
#include <QGridLayout>


QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();
    void addFrames(QString imagePath);
    void convertFrame(QImage image, QDataStream &out, quint32 delay);
    void addFramesIndexed(QString imagePath);
    void convertFrameIndexed(QImage image, QDataStream &out, quint32 delay);
private slots:
    void on_actionOpen_Gif_triggered();

    void on_actionUse_CLUT_smaller_files_triggered();

private:
    QByteArray m_qbaRawOutput;
    Ui::MainWindow *ui;
    QImageReader *m_reader;
    QImage * m_frame;
    QString m_path;
    QGridLayout *m_layout;
};
#endif // MAINWINDOW_H
