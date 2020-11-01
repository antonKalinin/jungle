# JUNGLE

<img src="https://github.com/antonKalinin/jungle/blob/master/assets/progress.gif?raw=true" width="480" />

1. [Window size & scale options](https://github.com/antonKalinin/jungle/tree/1-window-options)
1. [Parallax background](https://github.com/antonKalinin/jungle/tree/2-parallax-background)

### Split animated GIF to sequence of PNGs:

```
convert run.gif run_%01d.png
```

### To join 8 images into 1 sprite via ImageMagic:

```
montage run_[0-7].png -geometry '1x1+0+0<' -tile 8x1 -background none run.png
```

More about command parameters here: http://www.imagemagick.org/Usage/montage/
