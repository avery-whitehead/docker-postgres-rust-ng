import { Component, OnInit } from '@angular/core';
import { Observable, tap } from 'rxjs';
import { HomeService, Note } from './home.service';

@Component({
    selector: 'app-home',
    templateUrl: './home.component.html',
    styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {
    public notes$: Observable<Note[]>;

    constructor(private homeService: HomeService) {}

    ngOnInit() {
        this.notes$ = this.homeService.getNotes();
    }
}
