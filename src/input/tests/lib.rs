extern crate input;
extern crate serde_json;

use input::*;

#[test]
fn test_encode_decode() {
    let test = |input| {
        let encoded = serde_json::to_string(&input).unwrap();
        let decoded: Input = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, input);
    };

    test(Input::Button(ButtonArgs {
        state: ButtonState::Press,
        button: Button::Keyboard(Key::A),
        scancode: None,
    }));
    test(Input::Button(ButtonArgs {
        state: ButtonState::Release,
        button: Button::Keyboard(Key::A),
        scancode: None,
    }));
    test(Input::Move(Motion::MouseCursor(0.0, 0.0)));
    test(Input::Text("hello".into()));
    test(Input::Resize(0.0, 0.0));
    test(Input::Focus(true));
    test(Input::Cursor(true));
    test(Input::Close(CloseArgs));

    let test = |l| {
        let encoded = serde_json::to_string(&l).unwrap();
        let decoded: Loop = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, l);
    };
    test(Loop::Render(RenderArgs {
        width: 0.0,
        height: 0.0,
        draw_width: 0,
        draw_height: 0,
        ext_dt: 0.0,
    }));
    test(Loop::AfterRender(AfterRenderArgs));
    test(Loop::Update(UpdateArgs { dt: 0.0 }));
    test(Loop::Idle(IdleArgs { dt: 0.0 }));
}
