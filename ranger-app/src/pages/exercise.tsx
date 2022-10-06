import React from 'react';
import { useForm, SubmitHandler } from 'react-hook-form';
import axios from 'axios';

const styles = {
    container: {
        width: "80%",
        margin: "0 auto",
    },
    input: {
        width: "100%",
    },
};

type Exercise = {
    name: string,
    scenario: string,
};

const ExerciseForm = () => {

    const { register, handleSubmit, formState: { errors } } = useForm<Exercise>();
    const onSubmit: SubmitHandler<Exercise> = async exercise => {
        console.log(exercise);

        await axios.post('api/v1/exercise', JSON.stringify(exercise), {
            headers: {
                'Content-Type': 'application/json'
            },
        }).then((response) => {
            console.log(response.data);
        }).catch((error) => {
            console.log(error.response.data);
        });
    };

    return (
        <div style={styles.container} >
            <h4>Add exercise </h4>
            <form className="ExerciseForm" onSubmit={handleSubmit(onSubmit)} >
                <input placeholder="Exercise name" {...register("name", { required: true })} />
                <textarea {...register('scenario', { required: true })} placeholder="Scenario yaml" style={styles.input} />
                {errors.scenario && <p role="alert">{errors.scenario?.message}</p>}
                <button type="submit" > Submit </button>
            </form>
        </div>
    );

}

export default ExerciseForm;