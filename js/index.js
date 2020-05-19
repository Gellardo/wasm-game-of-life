import( "../pkg/index.js" ).then(wasm => game(wasm)).catch(console.error);

function game(wasm) {
    // const gameElement = document.getElementById("game")
    // const g = wasm.game()
    // const renderloop = async () => {
    //     gameElement.textContent = g.prettier_state()
    //     g.tick()
    //     await new Promise(r => setTimeout(r, 100));
    //     requestAnimationFrame(renderloop)
    // }
    // requestAnimationFrame(renderloop)
}
