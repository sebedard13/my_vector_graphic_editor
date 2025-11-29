import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppComponent } from "./app.component";
import { FontAwesomeModule } from "@fortawesome/angular-fontawesome";
import { ToolsBarComponent } from "./layout/tools-bar/tools-bar.component";
import { CanvasComponent } from "./scene/canvas/canvas.component";
import { ColorPickerComponent } from "./utilities/color-picker/color-picker.component";
import { EventsService } from "./scene/events.service";
import { MouseInfoComponent } from "./mouse-info/mouse-info/mouse-info.component";
import { MenuBarComponent } from "./layout/menu-bar/menu-bar.component";
import { SceneViewComponent } from "./scene/scene-view/scene-view.component";
import { NewSceneComponent } from "./new-scene/new-scene.component";
import { NumberInputComponent } from "./utilities/number-input/number-input.component";
import { FormsModule } from "@angular/forms";
import { SceneViewerComponent } from "./windows/scene-viewer/scene-viewer.component";
import { ToolsPropertiesBarComponent } from "./layout/tools-properties-bar/tools-properties-bar.component";

@NgModule({
    declarations: [
        AppComponent,
        ToolsBarComponent,
        CanvasComponent,
        MouseInfoComponent,
        MenuBarComponent,
        SceneViewComponent,
        NewSceneComponent,
    ],
    imports: [
        BrowserModule,
        FontAwesomeModule,
        FormsModule,
        SceneViewerComponent,
        ToolsPropertiesBarComponent,
        ColorPickerComponent,
        NumberInputComponent,
    ],
    providers: [EventsService],
    bootstrap: [AppComponent],
})
export class AppModule {}
