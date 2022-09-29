import React from 'react';
import { useForm } from 'react-hook-form';

const styles = {
    container: {
        width: "80%",
        margin: "0 auto",
    },
    input: {
        width: "100%",
    },
};

const ExerciseForm = () => {

    const { register, handleSubmit } = useForm();

    function onSubmit(data: any) {
        console.log(data.exercise);

        fetch('http://localhost:8080/', {
            method: 'POST',
            mode: 'cors',
            body: JSON.stringify(data.exercise)

        })

    }

    return (
        <div style={styles.container} >
            <h4>Add an exercise </h4>
            <form className="ExerciseForm" onSubmit={handleSubmit(onSubmit)} >
                <input {...register('exercise', { required: true })} placeholder="Exercise yml" style={styles.input} />
                <button type="submit" > Submit </button>
            </form>
        </div>
    );

}

export default ExerciseForm;