import * as color_imports from "@material-ui/core/colors";

const colors = Object.values(color_imports);

const pallette = function* ises(){
  var i = 0;
  while (true) {
    i += 1;
    const color = colors[i % colors.length];
    // @ts-ignore
    yield(color[500]);
  }
}()
