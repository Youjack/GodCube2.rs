import init, {search_wasm} from '../../pkg/god_cube2.js';

let config_form = document.querySelector('form');
let state_input = new Array();
let search_btn = document.querySelector('button');
let result_p = document.querySelector('p');

init().then(() => {
    for (let i = 0; i < 6; i++) {
        state_input.push(document.createElement('input'));
        state_input[i].setAttribute('type' , 'number'        );
        state_input[i].setAttribute('name' , 'int'           );
        state_input[i].setAttribute('min'  , '0'             );
        state_input[i].setAttribute('max'  , '20'            );
        state_input[i].setAttribute('value', (i*3).toString());
        config_form.appendChild(state_input[i]);
        config_form.appendChild(document.createTextNode('\n'));
    }

    search_btn.addEventListener('click', search);
});

function search() {
    search_btn.disabled = true;

    let state = new Array();
    for (let i = 0; i < 6; i++) {
        state.push(parseInt(state_input[i].value, 10));
    }

    try {
        let sol = search_wasm([config_form['algo'].value, state]);
        result_p.innerHTML = 
            // 'time: ' + time_ms + ' ms' + '<br>' +
            '#steps: ' + sol.length + '<br>' +
            'solution: ' + sol;
    } catch (error) {
        result_p.innerHTML = error;
    }

    search_btn.disabled = false;
}
