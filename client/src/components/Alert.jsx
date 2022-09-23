import "../App.css"

function Alert(props) {
  return (
    <div className="alert alert-error shadow-lg">
      <div>
        <span> {props.message}</span>
      </div>
    </div>
  )
}

export default Alert;