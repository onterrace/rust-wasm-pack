// Rust에서 라이브러리는 크레이트(crate)라고 합니다.
// 알 것 같나요? Cargo(화물) 는 배에 crate(상자) 들을 실어 나릅니다.
// 세번째 줄의 use 키워드는 라이브러리로부터 코드를 불러옵니다. 
// 이 경우, 우리는 wasm_bindgen::prelude 모듈 내에 있는 모든 것을 
// 불러오게 됩니다. 다음 섹션에서 이것들의 기능에 대해 다룰 것입니다.
use wasm_bindgen::prelude::*;


// '#[ ]' 안에 있는 것을 속성이라고 부르는데, 이것은 다음에 오는 구문을 
// 수정합니다. 이 경우에, 그 구문은 extern이며, Rust에게 외부에 정의된 
// 함수를 호출할 것임을 알립니다. 이 속성의 경우, "wasm-bindgen은 
// 이 함수들을 어떻게 찾을 것인지 알고 있다"고 알리는 것입니다.
#[wasm_bindgen]
extern {
    // 함수 시그니처는 Rust로 작성되어있습니다. alert 함수는 문자열 
    // 타입의 s 하나를 인자로서 받는다는 의미입니다.
    // 이것은 JavaScript에 의해 제공되는 alert 함수입니다
    pub fn alert(s: &str);
}

// Rust 함수를 JavaScript에 의해 호출될 수 있도록 함을 의미하며, extern 과는 반대 기능
#[wasm_bindgen]
pub fn greet(name: &str) {
    // tern 블록에서 요구한 alert 함수를 호출하여,
    alert(&format!("Hello, {}!", name));
}