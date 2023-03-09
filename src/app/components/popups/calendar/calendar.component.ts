import { CommonModule } from "@angular/common";
import {
  Component,
  HostListener,
  Inject,
  OnDestroy,
  OnInit,
  ViewEncapsulation,
} from "@angular/core";
import {
  DateAdapter,
  MAT_DATE_LOCALE,
  MatNativeDateModule,
} from "@angular/material/core";
import {
  MatCalendarView,
  MatDatepickerModule,
} from "@angular/material/datepicker";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import { CalendarProps } from "../../../../bindings/widget/datetime/CalendarProps";
import { StylesWatcherComponent } from "../../styles-watcher.component";

@Component({
  standalone: true,
  selector: "calendar",
  templateUrl: "./calendar.component.html",
  styleUrls: ["./calendar.component.scss"],
  imports: [MatDatepickerModule, MatNativeDateModule, CommonModule],
  encapsulation: ViewEncapsulation.None,
})
export class CalendarComponent
  extends StylesWatcherComponent
  implements OnInit, OnDestroy
{
  public selectedDate: Date = new Date();
  public startView: MatCalendarView = "month";
  public calendarClass?: string | null;
  private calendarUnlistenFn?: UnlistenFn;

  constructor(
    private _adapter: DateAdapter<any>,
    @Inject(MAT_DATE_LOCALE) private _locale: string
  ) {
    super();
  }

  public async ngOnInit(): Promise<void> {
    super.ngOnInit();
    this.calendarUnlistenFn = await listen(
      `${appWindow.label}_show`,
      async (event: any) => {
        const props = event.payload as CalendarProps | undefined;
        this._locale = props?.locale ?? this._locale;
        this._adapter.setLocale(this._locale);
        this.calendarClass = props?.popup?.class;
        await appWindow.show();
      }
    );
  }

  public async ngOnDestroy(): Promise<void> {
    await super.ngOnDestroy();
    this.calendarUnlistenFn && this.calendarUnlistenFn();
  }

  @HostListener("document:keydown.escape")
  protected async onEscape(): Promise<void> {
    await appWindow.hide();
  }
}
