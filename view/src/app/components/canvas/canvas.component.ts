import { OnInit, Component, ElementRef, AfterViewInit, ViewChild} from '@angular/core';
import * as wasm from 'wasm-vgc';
@Component({
  selector: 'app-canvas',
  templateUrl: './canvas.component.html',
  styleUrls: ['./canvas.component.scss']
})
export class CanvasComponent implements AfterViewInit{
 
  @ViewChild("canvas") canvas!: ElementRef<HTMLCanvasElement>;
  private resizeObserver: ResizeObserver | undefined;
  private ctx!: CanvasRenderingContext2D;

  ngAfterViewInit(): void {
    const width = this.canvas.nativeElement.offsetWidth;
    const height = this.canvas.nativeElement.offsetHeight;
    this.canvas.nativeElement.width = width;
    this.canvas.nativeElement.height = height;


    this.ctx = this.canvas.nativeElement.getContext('2d') as CanvasRenderingContext2D;

    this.resizeObserver = new ResizeObserver((entries) => {
      this.hideCanvas();
      this.canvas.nativeElement.removeAttribute("width");
      this.canvas.nativeElement.removeAttribute("height");
      setTimeout(() => {

        this.canvas.nativeElement.width = this.canvas.nativeElement.offsetWidth;
        this.canvas.nativeElement.height = this.canvas.nativeElement.offsetHeight;
      }, 1);
    });

    this.resizeObserver.observe(this.canvas.nativeElement.parentElement!);
    this.render()
  }

  public render(){
    wasm.render(this.ctx,this.canvas.nativeElement.width, this.canvas.nativeElement.height);
    this.showCanvas();
    window.requestAnimationFrame(this.render.bind(this))
  }


  public hideCanvas(){
    this.canvas.nativeElement.style.opacity = "0";
  }

  public showCanvas(){
    this.canvas.nativeElement.style.opacity = "1";
  }

 
}
