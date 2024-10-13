import "@analogjs/vitest-angular/setup-zone";
import "vitest-webgl-canvas-mock";

import {
    BrowserDynamicTestingModule,
    platformBrowserDynamicTesting,
} from "@angular/platform-browser-dynamic/testing";
import { getTestBed } from "@angular/core/testing";

getTestBed().initTestEnvironment(BrowserDynamicTestingModule, platformBrowserDynamicTesting());
