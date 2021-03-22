# qompo

proof of concept of web compiler using qoeur compiler and wasm compiler

*hello world*

```
use capsule script {
  val name : str = "world";
}

use capsule view {
  <h1>Hello {name}!</h1>
}
```

*styling css*

```
use capsule ui {
  .p {
    background: blue;
    color: white;
  }
}

use capsule view {
  <p>styled</p>
}
```

*styling: scss*

```
use capsule ui {
  .p {
    span {
      color: blue;
      font-size: 16px;
    }
  }
}

use capsule view {
  <h1>
    hello <span>world!</span>
  </h1>
}
```
