import { Component } from '@angular/core';
import * as wasm from 'wasm-vgc';
@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'view';

  public render(): string {
    let rtn = wasm.render();
    return rtn;

  }

}
