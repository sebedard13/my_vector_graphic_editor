import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppRoutingModule } from "./app-routing.module";
import { AppComponent } from "./app.component";
import { FontAwesomeModule } from "@fortawesome/angular-fontawesome";
import { ToolsBarComponent } from "./components/tools-bar/tools-bar.component";
import { CanvasComponent } from "./components/canvas/canvas.component";
import { ColorPickerComponent } from "./components/color-picker/color-picker.component";
import { EventsService } from "./events.service";
import { MouseInfoComponent } from "./mouse-info/mouse-info/mouse-info.component";
import { MenuBarComponent } from "./menu-bar/menu-bar.component";
import { SceneViewComponent } from "./components/scene-view/scene-view.component";
import { NewSceneComponent } from "./new-scene/new-scene.component";
import { NumberInputComponent } from "./number-input/number-input.component";
import { FormsModule } from "@angular/forms";

@NgModule({
    declarations: [
        AppComponent,
        ToolsBarComponent,
        CanvasComponent,
        ColorPickerComponent,
        MouseInfoComponent,
        MenuBarComponent,
        SceneViewComponent,
        NewSceneComponent,
        NumberInputComponent,
    ],
    imports: [BrowserModule, AppRoutingModule, FontAwesomeModule, FormsModule],
    providers: [EventsService],
    bootstrap: [AppComponent],
})
export class AppModule {}
