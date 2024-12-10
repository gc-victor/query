class Timer {
    #startTime;
    #endTime;

    #now() {
        return process.hrtime.bigint();
    }


    get startTime() {
        return (Number(this.#startTime) / 1000000).toFixed(3);
    }

    get endTime() {
        return (Number(this.#endTime) / 1000000).toFixed(3);
    }

    start() {
        this.#startTime = this.#now();
        return this;
    }

    end() {
        this.#endTime = this.#now();
        return this.#duration();
    }

    #duration() {
        return `${(Number(this.#endTime - this.#startTime) / 1000000).toFixed(3)}ms`;
    }
}

export const timer = new Timer();
