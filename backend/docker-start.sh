#!/bin/sh

mkdir /images
cp /default_images/* /images

exec ./app
