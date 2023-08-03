import { OnInit, Component, ElementRef, NgZone } from '@angular/core';
import { Application } from 'pixi.js';

@Component({
  selector: 'app-canvas',
  templateUrl: './canvas.component.html',
  styleUrls: ['./canvas.component.scss']
})
export class CanvasComponent implements OnInit{
  public app!: Application;
  constructor(private elementRef: ElementRef, private ngZone: NgZone) {

  }

  ngOnInit(): void {
    this.ngZone.runOutsideAngular(() => {
      this.app = new Application({});
    });
    // @ts-ignore
    this.elementRef.nativeElement.appendChild(this.app.view);
  }
}
