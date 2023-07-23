import {AfterViewInit, Component, EmbeddedViewRef} from '@angular/core';
import * as wasm from 'wasm-vgc';
import * as wasm_bg from 'wasm-vgc/wasm_vgc_bg.wasm';
@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements AfterViewInit{
   private ctx: CanvasRenderingContext2D;

  constructor() {
    let tmp = document.createElement("canvas") as HTMLCanvasElement
    this.ctx = tmp.getContext('2d') as CanvasRenderingContext2D;
  }

  ngAfterViewInit() {
    let canvas = document.querySelector("#canvas") as HTMLCanvasElement
    let temp_ctx = canvas.getContext('2d');
    if (!temp_ctx){
      console.error("Context is not 2d");
    }
    this.ctx = temp_ctx as CanvasRenderingContext2D;
    this.render()
  }

  public render(){
    let data_image = wasm.render();
    let img = new ImageData(data_image,512)
    this.ctx.putImageData(img, 0,0);
    window.requestAnimationFrame(this.render.bind(this))
  }

}
