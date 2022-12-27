# Rust를 WebAssembly로 컴파일하기

Rust 코드를  WebAssembly로 컴파일 하고 브라우저에서 사용하는 방법을 설명한다. 이 글은 [Rust를 WebAssembly로 컴파일하기](https://developer.mozilla.org/ko/docs/WebAssembly/Rust_to_wasm)에 있는 문서가 원본이므로 참고하라. 이 프로젝트는 Rust의 npm 패키지를 빌드하는 도구인 wasm-pack을 사용하여 npm 패키지를 빌드한다.  소스는 [Github](https://github.com/latteonterrace/rust-wasm-pack.git)을 참고한다. 


**개발환경**
* Node v19.3.0 
* cargo 1.65.0 (4bc8f24d3 2022-10-20)
* rustc 1.65.0 (897e37553 2022-11-02)


## wasm-pack 설치

다음을 실행하여 wasm-pack을 설치한다. 
```shell
cargo install wasm-pack
```
winddows에서 설치하다가 다음과 같은 에러가 발생했다. 

```shell
error: failed to run custom build command for `openssl-sys v0.9.80`
```

### 에러처리 방법(실패)

openssl을 설치한다. 
```shell
choco install openssl
```
마찬가지로 에러가 발생했다. OPENSSL_DIR 환경변수를 설정했지막 역시 컴파일 되지 않았다. 



### 에러처리 방법(성공)

wasm-pack을 설치하다가 오류가 나서 다음을 사용하여 다시 설치했다. [https://github.com/rustwasm/wasm-pack/issues/1026](https://github.com/rustwasm/wasm-pack/issues/1026)을 참고했다.  다음 방법은 openssl를 제외하고 빌드한다. 

```shell
cargo install wasm-pack --no-default-features
```

## 패키지 생성
다음을 실행하여 패키지를 생성한다. 
```shell
$ cargo new --lib hello-wasm
```
### lib.rs 
**hello-wasm/src/lib.rs**    
```rust
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
```

### 컴파일

**hello-wasm/src/Cargo.toml**    
```toml
[package]
name = "jirepos-hello-wasm"
version = "0.1.0"
edition = "2021"
authors = ["Jirepos <jirepos@gmail.com>"]

# [lib] 섹션은 패키지를 cdylib 형식으로 빌드할 것이라고 알린다. 
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"

# `wasm-opt` is on by default in for the release profile, but it can be
# disabled by setting it to `false`
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

의존하고 있는 wasm-bindgen을 설치하기 위해서 먼저 다음을 실행한다.
```shell
cd hello-wasm
cargo build
```
이제 다음을 실행하여 wasm 파일을 빌드한다.
```shell
wasm-pack build --scope mynpmusername
```


mynpmusername은 npm에 등록할 사용자 이름이다.

빌드하다가 에러가 나서 Cargo.toml에 다음을 추가했다.

```toml
# `wasm-opt` is on by default in for the release profile, but it can be
# disabled by setting it to `false`
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

이 명령어를 입력하면 많은 일이 일어난다. 

* Rust 코드를 WebAssembly로 컴파일 한다. 
* 그 WebAssembly 위에서 wasm-bindgen을 실행하여, WebAssembly를 npm이 이해할 수 있는 모듈로 감싸는 JavaScript 파일을 생성한다. 
* pkg 디렉터리를 만들고 JavaScript 파일과 WebAssembly 코드를 그 안으로 옮긴다. 
* Cargo.toml 을 읽고 동등한 package.json을 생성한다. 
* README.md 가 있다면 패키지로 복사한다. 


빌드가 끝났다면, pkg 디렉터리에 npm 패키지가 생성될 것이다. 

### npm에 배포하기 
npmjs에 배포하기 위해서는 [https://www.npmjs.com/](https://www.npmjs.com/)에 가입이 되어 있어야 한다. npm에 우리의 패키지를 배포하기 위해서는 다음과 같이 한다. 

```shell
cd pkg
npm publish --access=public
```


## 패키지 사용하기 
많은 사람들이 여러가지 번들러 도구를 사용해 npm패키지를 사용하는데, 이 튜토리얼에선 이들 중 하나인 webpack을 사용할 것입니다. 이것은 조금 복잡하고, 현실적인 사용 사례를 보여줄 것이다. 

pkg 와 hello-wasm 디렉터리를 빠져나가서, 다음처럼 site라는 이름의 새 디렉터리를 만들고 진입한다. 
```shell
cd ../..
mkdir site
cd site
```

### package.json 
package.json을 만들어 다음의 코드를 작성한다. 
```json
  {
      "scripts": {
        "serve": "webpack-dev-server"
      },
      "dependencies": {
        "@mynpmusername/hello-wasm": "^0.1.0"
      },
      "devDependencies": {
        "webpack": "^4.25.1",
        "webpack-cli": "^3.1.2",
        "webpack-dev-server": "^3.1.10"
      }
    }
```

dependencies 섹션에서 @ 뒤에 실제 npm 계정명을 넣어주어야 한다. 

다음은 Webpack을 설정해야 한다. webpack.config.js 파일을 만든 뒤, 다음 코드를 작성한다. 


```jsx
    const path = require('path');
    module.exports = {
      entry: "./index.js",
      output: {
        path: path.resolve(__dirname, "dist"),
        filename: "index.js",
      },
      mode: "development"
    };
```
그리고 HTML 파일도 필요합니다. index.html을 만들고, 다음 내용을 작성한다. 
```html
    <!DOCTYPE html>
    <html>
      <head>
        <meta charset="utf-8">
        <title>hello-wasm example</title>
      </head>
      <body>
        <script src="./index.js"></script>
      </body>
    </html>
```

마지막으로, HTML에서 참조되는 index.js를 만들어 다음 내용을 작성한다. 
```jsx
 const js = import("./node_modules/@yournpmusername/hello-wasm/hello_wasm.js");
    js.then(js => {
      js.greet("WebAssembly");
    });
```


파일들을 모두 만들었으니, 한번 보도록 하자.
```shell
npm install
npm run serve
```

webpack 실행 시 오류가 난다. 처리 방법은 [Webpack build failing with ERR_OSSL_EVP_UNSUPPORTED duplicate](https://stackoverflow.com/questions/69394632/webpack-build-failing-with-err-ossl-evp-unsupported)을 참고한다. 

아래 명령을 먼저 실행하고, npm run serve를 실행한다.

```shell
export NODE_OPTIONS=--openssl-legacy-provider
```

Power shell에서는 다음과 같이 한다. 
```shell
$env:NODE_OPTIONS="--openssl-legacy-provider" 
```

이제 웹브라우저에서 http://localhost:8080/ 에 접속하면 hello가 출력된다. 


