import "./App.css";
import Linkform from "./components/Linkform";

function App() {
  return (
    <div className="App">
      <div className="grid grid-cols-3 grid-rows-3 place-items-center h-screen">
        <div className="col-start-2 row-start-2 ">
          <Linkform />
        </div>
      </div>
    </div>
  );
}

export default App;
