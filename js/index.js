let target = document.getElementById('target');
let my_input = document.getElementById('my_input');

my_input.addEventListener('change', () => {
    let blob = my_input.files[0];
    let reader = new FileReader();
    reader.addEventListener('load', () => {
        import("../pkg/index.js").then((wasm) => {
            target.innerHTML = wasm.process_docx(new Uint8Array(reader.result));
        }).catch(console.error);
    });
    reader.readAsArrayBuffer(blob);
})

