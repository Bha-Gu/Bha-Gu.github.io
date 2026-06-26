let started = false;

export async function loadGame(canvasId, gameId) {
    if (started) {
        return;
    }
	
    const module = await import(
        `/assets/games/${gameId}/${gameId}.js`
    );

    await module.default();

    // if (module.start_game) {
    //     module.start_game(canvasId);
    // }

    // started = true;
}


export function stopGame() {

    console.log("canvas-remove")

    const canvas = document.getElementById("game-canvas");

    if (canvas) {
        canvas.remove();
    }
}
