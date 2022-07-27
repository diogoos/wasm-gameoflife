import('./pkg')
  .then((m) => {
    const app = new m.CellApp();

    // MARK: - RENDERING
    
    // throttle FPS (changes speed of animation)
    var FPS_THROTTLE = 1000.0 / 10.0;
    let lastDrawTime = -1;
    let isPaused = false;

    function render() {
      window.requestAnimationFrame(render);
      const currTime = Date.now();

      if (currTime >= lastDrawTime + FPS_THROTTLE) {
        lastDrawTime = currTime;

        if (!isPaused) app.update();
        app.draw();
      }
    }

    // MARK: - LIVE EDITING
    document.getElementById("canvas").addEventListener("click", (e) => {
      const rect = canvas.getBoundingClientRect()
      const x = e.clientX - rect.left
      const y = e.clientY - rect.top

      app.toggle_cell(x, y);
      app.draw(); // immediately toggle draw to make more responsive
    })


    // MARK: - BUTTONS
    const pauseBtn = document.getElementById("pauseGame");
    const stepBtn = document.getElementById("step");
    pauseBtn.addEventListener("click", () => {
      if (isPaused) {
        isPaused = false
        stepBtn.style.display = "none";
        pauseBtn.innerText = "Pause"
      } else {
        isPaused = true
        stepBtn.style.display = "inline-block";
        pauseBtn.innerText = "Play"
      }
    })

    stepBtn.addEventListener("click", () => { app.update(); })
    document.getElementById("clear").addEventListener("click", () => { app.clear_board() })
    document.getElementById("random").addEventListener("click", () => { app.randomize_board() })

    // MARK: - SETTINGS
    document.getElementById("framerate").onchange = function () {
      FPS_THROTTLE = 1000.0 / this.value;
      document.getElementById("fpscounter").innerText = `${this.value}fps`;
    }
    document.getElementById("invertbkg").onchange = () => { app.background_inverted = !app.background_inverted; }
    document.getElementById("showgrid").onchange = () => { app.show_grid = !app.show_grid; }

    document.getElementById("invertbkg").checked = false;
    document.getElementById("showgrid").checked = false;
    document.getElementById("framerate").value = 10;




    render();
  })
  .catch(console.error);