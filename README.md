# About
weval was made with the intent of having a portable and reliable eval function, the library can currently run on wasm (wasm-bindgen) and natively pretty much everywhere since the number of third party dependencies other than rust is almost absent.

as of now the library does not support unary operators and is not customizable.

as of version 0.1.0 the library can be considered usable in the sense that it won't crash in your face as long as you input a valid expression, but a syntax checker and unary operators support has to yet be introduced, 
this includes the lack of support for the minus sign in front of parentheses, that will make the eval crash, do not expect API stability yet though.

Later, if I find the time I also plan to implement support for higher precision decimals such as Decimal128 and maybe supoort for arbitrary operators (as functions).

# Build

1. [Install rust](https://www.rust-lang.org/tools/install)
2. Install wasm-pack: `cargo install wasm-pack`
3. Install cargo-make `cargo install cargo-make`
4. then build using `cargo make web` or `cargo make node`

# Usage

once built just get the js package, import then in your web project as a module like this

```js
import init, { eval_float64 } from './pkg-web/weval.js';
await init();
let num = eval_float64('33+454*(2**3)');
console.log(num);
```

you may also look into index.html for more insight, some functions like the tokenizer are also exposed, though it is more for educational purposes if you will

here how you may convert an expression from infix to postfix:

```js
import init, { eval_float64, tokenizer, infix_to_postfix_tokens } from './pkg-web/weval.js';
await init();
let expression = '(-34 + 2**3) * 3 + 2'

let tokens = tokenizer(expression);

//execute the shunting yard algorithm on the tokens
let postfixTokens = infix_to_postfix_tokens(tokens)

console.log('look mum, i made a postfix expression: ',postfixTokens.join(' '))
```

note that in order to have the demo, it being index.html work you have to build the library for web first.

have fun!
