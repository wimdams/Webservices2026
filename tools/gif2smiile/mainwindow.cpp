#include "mainwindow.h"
#include "ui_mainwindow.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);
    m_layout = new QGridLayout(ui->scrollWidget);
    m_path = QDir::homePath();
    ui->clut->setVisible(false);
}

void MainWindow::on_actionOpen_Gif_triggered()
{
    QString fileName = QFileDialog::getOpenFileName(this,
         tr("Open Gif"), m_path, tr("Gif Files (*.gif)"));
    if( !QFile::exists(fileName) )
        return;
    //Clear the ui
    foreach(QObject * o,ui->scrollWidget->children()){
        if(o->inherits("QLabel")){
            QLabel * del = static_cast<QLabel*>(o);
            delete del;
        }
    }

    QDir dir;
    m_path = dir.absoluteFilePath (fileName);
    if(ui->actionUse_CLUT_smaller_files->isChecked()){
        addFramesIndexed(fileName);
    }else{
        addFrames(fileName);
    }

}


void MainWindow::addFrames(QString imagePath)
{
    QString outPath(imagePath);
    outPath.replace(".gif",".smiile");
    QD << outPath << imagePath;
    QFile outFile(outPath);
    if(!outFile.open(QFile::WriteOnly|QFile::Truncate)){
        QMessageBox::warning(this, tr("Oops"),
                             tr("An error occured whilst trying to save the smiile file"),
                             QMessageBox::Ok ,
                             QMessageBox::Ok);
    }

    m_reader = new QImageReader(imagePath);
    m_frame = new QImage(m_reader->size(),QImage::Format_RGB32);

    QDataStream out(&outFile);
    out.setByteOrder(QDataStream::LittleEndian);

    out << 's' << 'm' << 'i' << 'i';
    out << ((char)0); //version 0
    out << ((char)0); //FLAGS 0 = RGB565
    out << ((quint16)0); //Lookup table length 0 = no CLUT
    out << (quint32)m_reader->size().width(); //First width then height TODO: Change this in spec
    out << (quint32)m_reader->size().height();
    out << (quint32)m_reader->imageCount(); //Frame count


    int i = 0;
    while(m_reader->read(m_frame)) {
        QLabel * newLbl = new QLabel(ui->scrollWidget);
        m_layout->addWidget(newLbl,i/5,i%5); i++;
        newLbl->setPixmap(QPixmap::fromImage(*m_frame));
        quint32 delay =  m_reader->nextImageDelay();
        convertFrame(*m_frame, out,delay);
    }
    delete m_reader;
    delete m_frame;
    outFile.close();
}

void MainWindow::convertFrame(QImage image, QDataStream &out, quint32 delay){
    //Over de pixels lopen
    for ( int y = 0; y < image.height(); y++ ){
        for ( int x = 0; x < image.width(); x++ ){

            QRgb simColor = image.pixel(x,y);
            // ARGB1555
            //int color = ((qAlpha(simColor)?0x8000:0) | (qRed(simColor) & 0xF8) << 7) | ((qGreen(simColor) & 0xF8) << 2) | ((qBlue(simColor) & 0xF8) >> 3);
            //RGB565
            int color = ((qRed(simColor) & 0xF8) << 8) | ((qGreen(simColor) & 0xFC) << 3) | ((qBlue(simColor) & 0xF8) >> 3);
            out << (quint16)color;
        }
    }
    out << (quint16)delay;
}


void MainWindow::addFramesIndexed(QString imagePath)
{
    QString outPath(imagePath);
    outPath.replace(".gif",".smiile");
    QD << outPath << imagePath;
    QFile outFile(outPath);
    if(!outFile.open(QFile::WriteOnly|QFile::Truncate)){
        QMessageBox::warning(this, tr("Oops"),
                             tr("An error occured whilst trying to save the smiile file"),
                             QMessageBox::Ok ,
                             QMessageBox::Ok);
    }

    m_reader = new QImageReader(imagePath);
    m_frame = new QImage(m_reader->size(),QImage::Format_RGB32);

    QDataStream out(&outFile);
    out.setByteOrder(QDataStream::LittleEndian);

    out << 's' << 'm' << 'i' << 'i';
    out << ((char)0); //version 0
    out << ((char)1); //FLAGS 1 = 8bit CLUT


    bool isFirstFrame = true;
    int i = 0;
    while(m_reader->read(m_frame)) {
        QLabel * newLbl = new QLabel(ui->scrollWidget);
        m_layout->addWidget(newLbl,i/5,i%5); i++;
        newLbl->setPixmap(QPixmap::fromImage(*m_frame));
        quint32 delay =  m_reader->nextImageDelay();
        QImage indexedImage = m_frame->convertToFormat(QImage::Format_Indexed8);
        if(isFirstFrame){
            //We assume the pallete is the same for all frames!
            isFirstFrame = false;
            int index_cntr;
            QByteArray buffer;
            indexedImage.setColorCount(3);
            QList<QRgb> CLUT = indexedImage.colorTable();
            ui->clut->setCLUT(CLUT);
            for(index_cntr = 0; index_cntr < CLUT.length(); index_cntr++){
                if(index_cntr!=0 &&  CLUT.at(index_cntr) == 0 &&  CLUT.at(index_cntr-1) == 0){
                    //if there are multiple black values the CLUT the rest of the CLUT doesn' contain valid data
                    index_cntr--; //this is not a valid index
                    QD << index_cntr;
                    break;
                }
                QRgb simColor = CLUT.at(index_cntr);
                quint16 color = ((qRed(simColor) & 0xF8) << 8) | ((qGreen(simColor) & 0xFC) << 3) | ((qBlue(simColor) & 0xF8) >> 3);
                //Litle endian append
                buffer.append((quint8) (color&0xFF));
                buffer.append((quint8) ((color>>8)&0xFF));

            }
            out << ((quint16) (0xFFFF & index_cntr)); //Lookup table length 0 = no CLUT
            out << (quint32)m_reader->size().width(); //First width then height TODO: Change this in spec
            out << (quint32)m_reader->size().height();
            out << (quint32)m_reader->imageCount(); //Frame count
            out << buffer;
        }
        convertFrameIndexed(indexedImage, out,delay);
    }
    delete m_reader;
    delete m_frame;
    outFile.close();
}

void MainWindow::convertFrameIndexed(QImage image, QDataStream &out, quint32 delay){
    //Over de indexen lopen
    for (int y = 0; y < image.height(); ++y) {
        quint8 *line = reinterpret_cast<quint8*>(image.scanLine(y));
        for (int x = 0; x < image.width(); ++x) {
            quint8 index = line[x];
            out << index;
        }
    }

    // for ( int y = 0; y < image.height(); y++ ){
    //     for ( int x = 0; x < image.width(); x++ ){
    //         QRgb simColor = image.pixel(x,y);
    //         if((quint32) simColor > 255){
    //             QD << "Error indexed color can't be higher than 255" << x << y << simColor;
    //         }
    //         out << (quint8)(simColor & 0xFF);
    //     }
    // }
    out << (quint16)delay;
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::on_actionUse_CLUT_smaller_files_triggered()
{
    bool isIndexed = ui->actionUse_CLUT_smaller_files->isChecked();
    ui->actionUse_CLUT_smaller_files->setIcon(isIndexed?QIcon(":/res/8bit_indexed.png"):QIcon(":/res/16bit_non-indexed.png"));
    ui->clut->setVisible(isIndexed);
}

