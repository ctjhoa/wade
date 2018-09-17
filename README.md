# Web Assembly Document Editor (experimental)

*This is an experimental project you should not use it in production in any case.*

The goal of this project is to provide an alternative to JS Rich Text Editor (eg. Tinymce, ace editor ...) using Rust & Wasm technologies.
The main idea is to have only a representation of the document inside the [DOM](https://developer.mozilla.org/fr/docs/Web/API/Document_Object_Model)
and apply transformation to the document through JS events/actions.

## Usage

Wade is a library, not an application so you have to use it through examples if you want to experiment it.

```
$ cd examples/hello
$ cargo web start
```

