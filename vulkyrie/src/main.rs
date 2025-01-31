use vraylib::*;

fn main() {
    unsafe {
        InitWindow(800, 600, b"Hello, Raylib!\0".as_ptr() as *const i8);

        while !WindowShouldClose() {
            BeginDrawing();
            // ClearBackground(Color::);
            // DrawText(b"Hello, World!\0".as_ptr() as *const i8, 10, 10, 20, DARKGRAY);
            EndDrawing();
        }
    }
}
