import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { ToolsBarComponent, ButtonDirective } from './components/tools-bar/tools-bar.component';
import {CanvasComponent} from "./components/canvas/canvas.component";
import {RightBarComponent} from "./components/right-bar/right-bar.component";
import { ColorPickerComponent } from './components/right-bar/color-picker/color-picker.component';


@NgModule({
    declarations: [
        AppComponent,
        ToolsBarComponent,
        ButtonDirective,
        CanvasComponent,
        RightBarComponent,
        CanvasComponent,
        RightBarComponent,
        ColorPickerComponent
    ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    FontAwesomeModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {


}