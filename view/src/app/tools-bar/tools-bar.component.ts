import {Component, ContentChildren, Directive, QueryList} from '@angular/core';
import { faCoffee, faWheelchairMove} from "@fortawesome/free-solid-svg-icons";


@Directive({selector: 'button'})
export class ButtonDirective {
}

@Component({
  selector: 'app-tools-bar',
  templateUrl: './tools-bar.component.html',
  styleUrls: ['./tools-bar.component.scss']
})
export class ToolsBarComponent {
  faCoffee = faCoffee;
  faMove = faWheelchairMove

  @ContentChildren(ButtonDirective) contentChildren!: QueryList<ButtonDirective>;

  constructor() { }

  ngOnInit(): void {
  }

  ngAfterViewInit(){

  }


  onclick(){

  }
}
