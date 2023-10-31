import validator from 'validator';
import type {EmailForm} from 'src/models/email';
import type {AdUser} from 'src/models/groups';
import nunjucks from 'nunjucks';

export const validateEmailAddresses = (emails: string[]) =>
  emails.filter(email => !validator.isEmail(email));

export const validateEmailForm = (email: EmailForm) => {
  const invalidEmailAddresses = [
    ...validateEmailAddresses(email.toAddresses),
    ...validateEmailAddresses(email.replyToAddresses ?? []),
    ...validateEmailAddresses(email.ccAddresses ?? []),
    ...validateEmailAddresses(email.bccAddresses ?? []),
  ];

  return {
    invalidEmailAddresses,
    isValid: invalidEmailAddresses.length === 0,
  };
};

export const removeUnnecessaryEmailAddresses = (email: EmailForm) => {
  email.replyToAddresses = [];
  email.toAddresses = [];
  email.ccAddresses = [];
  email.bccAddresses = [];
};

export const prepareEmail = (
  email: EmailForm,
  exerciseName: string,
) => ({
  ...email,
  subject: nunjucks.renderString(email.subject, {
    exerciseName,
  }),
  body: nunjucks.renderString(email.body, {
    exerciseName,
  }),
});

export const prepareEmailForDeploymentUser = (
  email: EmailForm,
  exerciseName: string,
  deploymentName: string,
  user: AdUser,
) => ({
  ...email,
  toAddresses: [user.email ?? ''],
  subject: nunjucks.renderString(email.subject, {
    exerciseName,
    deploymentName,
    participantFirstName: user.firstName,
    participantLastName: user.lastName,
    participantEmail: user.email,
  }),
  body: nunjucks.renderString(email.body, {
    exerciseName,
    deploymentName,
    participantFirstName: user.firstName,
    participantLastName: user.lastName,
    participantEmail: user.email,
  }),
  userId: user.id,
});

export const preventDefaultOnEnter = (event: React.KeyboardEvent<HTMLFormElement>) => {
  if (event.key === 'Enter' && event.target instanceof HTMLInputElement) {
    event.preventDefault();
  }
};
