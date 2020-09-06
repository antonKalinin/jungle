### To join 8 images into 1 sprite via ImageMagic:

```
montage run_[0-7].png -geometry '1x1+0+0<' -tile 8x1 -background none run.png
```

More about command parameters here: http://www.imagemagick.org/Usage/montage/
