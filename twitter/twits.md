1.  Here goes lazy thread about how I am learning #rustlang and #gamedev by making platformer game with @BevyEngine.
    #bevyengine #rustgamedev

2.  Short intro: being a JS/TS web dev for almost all my career last year I decided to taste some Rust. My learning path: read the book, done a couple of tutorials, was looking for something more solid. Game is a good choice for such purpose.

3.  Having kind of huge cemetery of pet projects I knew that if I start creating assets by my own, it would be another headstone, so I found this super nice asset pack https://jesse-m.itch.io/jungle-pack by @Jsf23art.

4.  Next is game engine: I wanted it to be as thin as possible to try implement more by myself. A month ago GitHub trends showed me @BevyEngine. https://bevyengine.org/

5.  Bevy is ECS based data driven engine. I didn't know anything about Entity Component System paradigm before and probably still missing something now, but it gives you lots of freedom how to do things.

6.  So components are simple rust structs, that could be grouped into entities and passed to systems to run your game logic. This is super simplified but you can get more from https://bevyengine.org/learn/book/getting-started/ecs/ and https://github.com/bevyengine/bevy/blob/master/examples/ecs/ecs_guide.rs

7.  I tried to start with more fun parts so I wouldn't lose interest in the beginning. Like parallax background: I thought it should be easy, press arrow key and move sprites with different speed. The closer image the faster it moves.
