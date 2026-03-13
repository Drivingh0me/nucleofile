#include "outFiles.h"

#include <stdio.h>
#include <stdlib.h>

#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"

unsigned char* gen_image(int width, int height, int channels)
{
    int fileSize = width * height * channels;
    unsigned char *img = NULL;
    img = (unsigned char*)malloc(fileSize * sizeof(unsigned char));

    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            img[i * width * channels + (channels * j)] = 255;
            img[i * width * channels + (channels * j) + 1] = 0;
            img[i * width * channels + (channels * j) + 2] = 0;
            img[i * width * channels + (channels * j) + 3] = 255;
        }
    }

    return img;
}

int make_white_png(int width, int height)
{
    int channels = 4;

    /* create image data */
    unsigned char *img = gen_image(width, height, channels);
    if (img == NULL) {
        return 1;
    }

    /* Export image with stb_image */
    stbi_write_png("test3.png", width, height, channels, img, width * channels);

    free(img);

    return 0;
}



