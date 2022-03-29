import 'package:flowy_sdk/protobuf/flowy-grid/date_type_option.pb.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'dart:async';

part 'date_bloc.freezed.dart';

class DateTypeOptionBloc extends Bloc<DateTypeOptionEvent, DateTypeOptionState> {
  DateTypeOptionBloc({required DateTypeOption typeOption}) : super(DateTypeOptionState.initial(typeOption)) {
    on<DateTypeOptionEvent>(
      (event, emit) async {
        event.map(
          didSelectDateFormat: (_DidSelectDateFormat value) {
            state.typeOption.dateFormat = value.format;
            emit(state);
          },
          didSelectTimeFormat: (_DidSelectTimeFormat value) {
            state.typeOption.timeFormat = value.format;
            emit(state);
          },
        );
      },
    );
  }

  @override
  Future<void> close() async {
    return super.close();
  }
}

@freezed
class DateTypeOptionEvent with _$DateTypeOptionEvent {
  const factory DateTypeOptionEvent.didSelectDateFormat(DateFormat format) = _DidSelectDateFormat;
  const factory DateTypeOptionEvent.didSelectTimeFormat(TimeFormat format) = _DidSelectTimeFormat;
}

@freezed
class DateTypeOptionState with _$DateTypeOptionState {
  const factory DateTypeOptionState({
    required DateTypeOption typeOption,
  }) = _DateTypeOptionState;

  factory DateTypeOptionState.initial(DateTypeOption typeOption) => DateTypeOptionState(typeOption: typeOption);
}