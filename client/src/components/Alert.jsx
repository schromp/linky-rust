import "../App.css"

function Alert(props) {
  return (
    <div className="alert alert-error shadow-lg mt-4">
      <div>
        <span> {props.message}</span>
      </div>
    </div>
  )
}

export default Alert;