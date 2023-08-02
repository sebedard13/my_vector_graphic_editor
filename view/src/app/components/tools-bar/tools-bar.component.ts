import {Component, Directive} from '@angular/core';
import {Button} from "../../interface/Button";
import {toolsbarSvgBtn} from "../../config/toolsbar-svg-btn";


@Directive({selector: 'button'})
export class ButtonDirective {
}

@Component({
  selector: 'app-tools-bar',
  templateUrl: './tools-bar.component.html',
  styleUrls: ['./tools-bar.component.scss']
})
export class ToolsBarComponent {
  buttons: Button[] = toolsbarSvgBtn

  constructor() { }

  ngOnInit(): void {
  }

  ngAfterViewInit(){

  }


  onclick(){

  }
}
