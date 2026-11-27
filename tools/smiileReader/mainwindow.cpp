#include "mainwindow.h"
#include "ui_mainwindow.h"

#include <QFileDialog>
#include <QMessageBox>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    m_path = QDir::homePath();
    m_layout = new QGridLayout(ui->scrollWidget);
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::on_actionOpen_Gif_triggered()
{
    QString fileName = QFileDialog::getOpenFileName(this,
        tr("Open smiile"), m_path, tr("Smiile Image Files (*.smiile)"));
    if( !QFile::exists(fileName) )
        return;

    QDir dir;
    m_path = dir.absoluteFilePath (fileName);

    //Clear the ui
    foreach(QObject * o,ui->scrollWidget->children()){
        if(o->inherits("QLabel")){
            QLabel * del = static_cast<QLabel*>(o);
            delete del;
        }
    }

    QFile inFile(fileName);
    if(!inFile.open(QFile::ReadOnly)){
        QMessageBox::warning(this, tr("Oops"),
                             tr("An error occured whilst trying to open the smiile file"),
                             QMessageBox::Ok ,
                             QMessageBox::Ok);
    }

    QDataStream in(&inFile);
    in.setByteOrder(QDataStream::LittleEndian);

    char magicnumber[5];
    in >> magicnumber[0] >> magicnumber[1] >> magicnumber[2] >> magicnumber[3];
    char version;
    in >> version;
    char flags; //FLAGS 0 = RGB565 1=indexed8
    in >> flags;
    quint16 lenCLUT;
    in >> lenCLUT;
    quint32 img_width;
    in >> img_width;
    quint32 img_height;
    in >> img_height;
    quint32 framecount;
    in >> framecount;

    QString f = tr("Magicnumber: %1\n").arg(QByteArray(magicnumber,4));
    f.append(tr("version: %1\n").arg((int)version));
    f.append(tr("flags: %1\n").arg((int)flags));
    f.append(tr("CLUT length: %1\n").arg(lenCLUT));
    f.append(tr("Image width: %1\n").arg(img_width));
    f.append(tr("Image height: %1\n").arg(img_height));
    f.append(tr("framecount: %1\n").arg(framecount));

    ui->lblFeedback->setText(f);
    QList<QRgb> colors;
    QRgb color;
    quint16 rcolor;
    if(flags == 1 && lenCLUT>0){
        //Read CLUT
        for(int i=0; i < 256; i++){
            if(i < lenCLUT){
                in >> rcolor;
                color = qRgb((rcolor & 0xf800)>>(11-3), (rcolor & 0x07E0)>>(5-2), (rcolor & 0x001F)<<3); //todo copy msb to lsb for each color
                colors.append(color);
            }else{
                colors.append(Qt::black);
            }
        }
        ui->clut->setCLUT(colors);

        QImage frame(img_width,img_height,QImage::Format_RGB888);
        for(quint32 y = 0; y < img_height; y++){
            for(quint32 x = 0; x < img_width; x++){
                quint8 index;
                in >> index;
                frame.setPixel(x,y,colors.at(index));
                if(y == 0 && x < 20){
                    QD << index;
                }

            }
        }
        QLabel * newLbl = new QLabel(ui->scrollWidget);
        m_layout->addWidget(newLbl,0,0);
        newLbl->setPixmap(QPixmap::fromImage(frame));
        quint16 delay;
        in >> delay;
        QD << "delay = " << delay;
    }




}

