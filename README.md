# photo_mosaic
A toy project in Rust to generate Photo Mosaic.

## Project Components
- data: photos .png
- data representation
  - a) average pixel value per channel
  - b) feature extraction via MobileNet-v2 ..?
- nearest neighbor search
  - Faiss
- output: generated image in png

## core functions
1) build DB with ingredient pngs & generate feature vectors
2) build Faiss Index
3) find the nearest image for each small patch of the source image
4) generate photo mosaic with the search results