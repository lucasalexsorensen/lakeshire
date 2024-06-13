import React from 'react'
import { Link } from 'react-router-dom'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faArrowLeft } from '@fortawesome/free-solid-svg-icons'

export default function Navigation () {
    return (
        <div className="pl-2 pt-2 mb-2">
            <Link to={`/`}>
                <FontAwesomeIcon icon={faArrowLeft} size="xl" />
            </Link>
        </div>
    )
}
