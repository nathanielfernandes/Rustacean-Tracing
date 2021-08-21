# Rustacean-Tracing

## A `WIP 🏗️` Ray-Tracing engine written in rust

<div style="width:600px; margin-bottom: 4rem">
This ray-tracing engine runs entirely on the cpu and is shape/object agnostic.

It has the option to render the scene regulary or in threaded slabs, gaining on average a `5x` performance boost.

#### Next Steps

- ~Specular Highlights~
- ~Reflection~
- Global illumination
- Transparency/Refraction
- Texture mapping
- A few more shapes

</div>

## Sample Rendered Scenes

Some sample scenes rendered with this engine.

### [Render 1]("./samples/sample_2.png")

<div style="width:600px; margin-bottom: 4rem">

<a href="./samples/sample_2.png">
    <img src="./samples/sample_2.png" >
</a>
   
<div style="text-align: center; font-size: 14px; margin-bottom: 1rem">
    1 shape, 7 lights | <code>7680x4320</code>
</div>
    
<br>

| Resolution  | Max Light Bounces | Render Time | Render Time (threaded) |
| :---------: | :---------------: | :---------: | :--------------------: |
| `1920x1080` |       `14`        |   `703ms`   |        `170ms`         |
| `3840x2160` |       `14`        |   `3.4s`    |        `662ms`         |
| `7680x4320` |       `14`        |  `12.48s`   |        `2.71s`         |

</div>

### [Render 2]("./samples/sample_1.png")

<div style="width:600px; margin-bottom: 4rem">

<a href="./samples/sample_1.png">
    <img src="./samples/sample_1.png" >
</a>
   
<div style="text-align: center; font-size: 14px; margin-bottom: 1rem">
    4 shapes, 2 lights | <code>7680x4320</code>
</div>
    
<br>

| Resolution  | Max Light Bounces | Render Time | Render Time (threaded) |
| :---------: | :---------------: | :---------: | :--------------------: |
| `1920x1080` |       `14`        |   `324ms`   |         `77ms`         |
| `3840x2160` |       `14`        |  `1.376s`   |        `461ms`         |
| `7680x4320` |       `14`        |   `5.86s`   |        `1.16s`         |

</div>

### [Render 3]("./samples/sample_3.png")

<div style="width:600px; margin-bottom: 4rem">

<a href="./samples/sample_3.png">
    <img src="./samples/sample_3.png" >
</a>
   
<div style="text-align: center; font-size: 14px; margin-bottom: 1">
    3 shapes, 2 lights | <code>7680x4320</code>
</div>

<br>
    
| Resolution  | Max Light Bounces | Render Time | Render Time (threaded) |
| :---------: | :---------------: | :---------: | :--------------------: |
| `1920x1080` |       `28`        |  `7.221s`   |        `1.74s`         |
| `3840x2160` |       `28`        |  `28.93s`   |        `6.30s`         |
| `7680x4320` |       `28`        |  `121.55s`  |        `33.57s`        |

The max number of light bounces greatly affects the speed of the render.

</div>
