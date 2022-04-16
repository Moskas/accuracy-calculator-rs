# Terminal osu!mania accuracy calculator made in Rust

-------------------------------------------------------------------------------

Simple terminal accuracy and ratio calculator for osu!mania.
You can execute the application without any parameters and manually insert all the data. <br/>

![example screenshot](./example.png)<br/>

You can also launch the application with arguments, currently implemented arguments are:
- h - prints out available arguments and how to use them
- v - application version
- j - used to pipeing the judgments in format `-j "300g,300,200,100,50,miss"`

## Building
```
$ git clone https://github.com/Moskas/accuracy-calculator-rs.git
$ cd accuracy-calculator-rs
$ cargo build --release
```

