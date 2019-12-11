enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(&'static str),
    Click { x:i32, y:i32},
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page Loaded"),
        WebEvent::PageUnload => println!("Page Unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed '{}'.",c),
        WebEvent::Paste(str) => println!("pasted \"{}\"",str),
        WebEvent::Click{x, y} => {
            println!("Clicked at x={},y={}",x,y);
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text");
    let click = WebEvent::Click {x:20, y:40};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}