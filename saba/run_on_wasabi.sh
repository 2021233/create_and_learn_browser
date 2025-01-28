#!/bin/bash -xe

HOME_PATH=$PWD
TARGET_PATH=$PWD"/build"
OS_PATH=$TARGET_PATH"/wasabi"
APP_NAME="saba"
MAKEFILE_PATH=$HOME_PATH"/Makefile"

if [ -d $TARGET_PATH ]
then
  echo $TARGET_PATH" exists"
else
  echo $TARGET_PATH" dosen't exist"
  mkdir $TARGET_PATH
fi

if [ -d $OS_PATH ]
then
  echo $OS_PATH" exists"
  echo "pulling new changes..."
  cd $TARGET_PATH
  git pull origin for_saba
else
  echo $OS_PATH" dosen't exist"
  echo "cloneing wasabi project..."
  cd $TARGET_PATH
  git clone --branch for_saba https://github.com/hikalium/wasabi.git
fi

cd $HOME_PATH

if [ ! -f $MAKEFILE_PATH ]
then
  echo "downloading Makefile..."
  wget https://raw.githubusercontent.com/hikalium/wasabi/main/external_app_template/Makefile
fi

make build 
$OS_PATH/scripts/run_with_app.sh ./terget/x86_64-unknown-none/release/$APP_NAME
