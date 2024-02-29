FROM jupyter/scipy-notebook
USER root

# 必要なライブラリをインストール
RUN apt-get update && apt-get install -y \
    libgdal-dev \
    gdal-bin \
    && rm -rf /var/lib/apt/lists/* 


# GDALのパスを設定
ENV CPLUS_INCLUDE_PATH=/usr/include/gdal
ENV C_INCLUDE_PATH=/usr/include/gdal

RUN pip3 install geopandas==0.14.3 psycopg2-binary==2.9.9 geoalchemy2==0.14.4
