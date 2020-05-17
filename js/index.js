import( "../pkg/index.js" ).then(wasm => game(wasm)).catch(console.error);

function game(wasm) {
    const gameElement = document.getElementById("game")
    const g = wasm.game()
    const renderloop = async () => {
        console.log("render")
        gameElement.textContent = wasm.to_string(g)
        wasm.tick(g)
        await new Promise(r => setTimeout(r, 100));
        requestAnimationFrame(renderloop)
    }
    requestAnimationFrame(renderloop)
}
