const language = {
  homepage: 'Welcome to Ranger!',
  appName: 'Ranger',
  exercises: {
    title: 'Exercises',
    add: 'Add exercise',
    addingSuccess: 'Exercise {{exerciseName}} added',
    addingFail: 'Failed to add the exercise',
    name: 'Exercise name',
    scenarioSDL: 'Scenario SDL',
    deleteSuccess: 'Exercise {{exerciseName}} deleted',
    deleteFail: 'Failed to delete the exercise {{exerciseName}}',
  },
  deployments: {
    title: 'New deployment',
    add: 'Add Deployments',
    addingSuccess: 'Deployment {{newDeploymentName}} added',
    addingFail: 'Failed to add the deployment',
    sdlMissing: 'Exercise must have an sdl-schema',
    deleteSuccess: 'Deployment {{deploymentName}} deleted',
    deleteFail: 'Failed to delete the deployment',
    form: {
      group: {
        title: 'Deployment group',
        required: 'Deployment group is required',
      },
      name: {
        title: 'Deployment name',
        required: 'Deployment name is required',
      },
      count: {
        title: 'Deployment count',
        required: 'Deployment count is required',
      },
    },
  },
  menu: {
    home: 'Home',
    exercises: 'Exercises',
    logout: 'Logout',
    greeting: 'Hi, {{username}}!',
  },
  common: {
    submit: 'Submit',
    delete: 'Delete',
    deleting: 'Deleting',
    add: 'Add',
    virtualMachines: 'Virtual Machines',
    switches: 'Switches',
    templates: 'Templates',
  },
  emails: {
    emailLog: 'Email log:',
    status: 'Status',
    timestamp: 'Timestamp',
    toEntity: 'To Entity',
    from: 'From',
    to: 'To',
    replyTo: 'Reply to',
    subject: 'Subject',
    bcc: 'Bcc',
    cc: 'Cc',
    body: 'Body',
    send: 'Send email',
    sendingSuccess: 'Email sent',
    sendingFail: 'Error trying to send the email: {{errorMessage}}',
    sendingFailWithoutMessage: 'Failed to send the email',
    form: {
      from: {
        title: 'From',
      },
      to: {
        title: 'To',
        required: 'Receiver address is required',
      },
      replyTo: {
        title: 'Reply to',
      },
      cc: {
        title: 'Cc',
      },
      bcc: {
        title: 'Bcc',
      },
      subject: {
        title: 'Email subject',
        required: 'Email subject is required',
      },
      body: {
        title: 'Email body',
        required: 'Email body is required',
      },
      emailPlaceholder:
      'Enter email address, multiple addresses separated by comma',
    },
  },
};

export default language;
