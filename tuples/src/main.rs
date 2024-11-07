struct RGB(u8, u8, u8); // elements dont have names in tuple structs
struct CMYK(u8, u8, u8, u8);
struct UnitStruct; // () is empty
// can be used for traits somehow

fn main() {
    // Color tuples
    let rgb_color = RGB(255, 0, 0);
    let cmyk_color = CMYK(0, 255, 0, 100);
}
